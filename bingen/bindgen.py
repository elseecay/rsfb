import json
import re
from collections import namedtuple
from stringcase import snakecase


Ctype = namedtuple('Ctype', ('base', 'ptr', 'const'))
Cfunc = namedtuple('Cfunc', ('name', 'rtype', 'args', 'const'))


class TypeTranslater:

    @staticmethod
    def default(ctype):
        rsbase = cxx2rust["default"][ctype.base]
        if ctype.ptr:
            if ctype.const:
                return f'CPtr<{rsbase}>'
            else:
                return f'Ptr<{rsbase}>'
        return rsbase

    @staticmethod
    def fbinterface(ctype):
        rsbase = cxx2rust["fbinterface"][ctype.base]
        if ctype.ptr:
            if ctype.const:
                return f'{rsbase}CPtr'
            else:
                return f'{rsbase}Ptr'
        raise Exception('non-pointer fbinterface type')


def remove_template(content):
    r = re.compile(r"template[\s]*<[\w\s]+>[\s]*")
    return r.sub("", content)


def remove_virtual(content):
    r = re.compile(r"\svirtual\s")
    return r.sub("", content)


def remove_comments(content):
    r = re.compile(r"//.*\n")
    content = r.sub('', content)
    r = re.compile(r"/\*((.)|(\s))*\*/")
    content = r.sub('', content)
    return content


def remove_function_body(content):
    counter = 0
    parts = []
    cur_begin = None
    for i, c in enumerate(content):
        if c == '{':
            counter += 1
            if counter == 1:
                cur_begin = i
        elif c == '}':
            counter -= 1
            if counter < 0:
                raise Exception('WTF')
            if not counter:
                parts.append((cur_begin, i + 1))
    cur_begin = 0
    result = ''
    for b, e in parts:
        result += content[cur_begin:b].strip() + ';'
        cur_begin = e
    return result


def only_spaces(content):
    r = re.compile(r"\s")
    return r.sub(" ", content)


def remove_mult_spaces(content):
    r = re.compile(r" {2,}")
    return r.sub(" ", content)


def clear(content):
    content = remove_function_body(content)
    content = remove_comments(content)
    content = remove_template(content)
    content = only_spaces(content)
    content = remove_mult_spaces(content)
    content = remove_virtual(content)
    return content


def funclines(content):
    lst = [s for s in content.split(';')[:-1]]
    return lst


def build_cfunc(line):
    r = re.compile(r'\w+\(')
    mfname = r.search(line)
    name = line[mfname.start():mfname.end() - 1]
    rtype = line[:mfname.start()].strip()

    r = re.compile(r'\)\s*const')
    mconst = r.search(line)
    if mconst:
        const = True
        line = line[mfname.end():mconst.start()]
    else:
        const = False
        line = line[mfname.end():-1]

    args = []
    r = re.compile(r'\w+$')
    splist = line.split(',')
    if len(splist) == 1 and not len(splist[0]):
        splist = []
    for arg in splist:
        arg = arg.strip()
        m = r.search(arg)
        args.append({"type": build_ctype(arg[:m.start() - 1]), "name": arg[m.start():m.end()]})
    return Cfunc(name, build_ctype(rtype), args, const)


def build_ctype(line):
    if line.count('*') > 1:
        raise NotImplementedError('Multiple type pointer')
    if line.count('('):
        raise NotImplementedError('Function pointer')
    if line.find('const') > line.find('*'):
        raise NotImplementedError('Const pointer?')
    ptr, const = False, False
    if line.count('*'):
        line = line.replace('*', '')
        ptr = True
        if line.count('const'):
            line = line.replace('const', '')
            const = True
        line = line.strip()
    base = line
    return Ctype(base, ptr, const)


def ctype_tostr(ctype):
    return f'{"const " if ctype.const else ""}{ctype.base}{"*" if ctype.ptr else ""}'


def build_cfunc_string(cfunc):
    ret_str = ctype_tostr(cfunc.rtype)
    name_str = f'{prefix}_{snakecase(cfunc.name)}'
    args_str = f'{"const " if cfunc.const else ""}{config["classname"]}* self, '
    for a in cfunc.args:
        args_str += f'{ctype_tostr(a["type"])} {a["name"]}, '
    args_str = args_str[:-2]
    call_args_str = ''
    for a in cfunc.args:
        call_args_str += f'{a["name"]}, '
    call_args_str = call_args_str[:-2]
    return '%s %s(%s)\n{\n\treturn self->%s(%s);\n}' % (ret_str, name_str, args_str, cfunc.name, call_args_str)


def get_ctype_category(ctype):
    for cat in cxx2rust:
        if ctype.base in cxx2rust[cat]:
            return cat
    raise Exception(f'Unknown type \'{ctype.base}\'')


def build_rtype_string(ctype):
    cat = get_ctype_category(ctype)
    fn = getattr(TypeTranslater, cat)
    return fn(ctype)


def build_rtype_trait_string(ctype):
    cat = get_ctype_category(ctype)
    if cat == 'fbinterface':
        if not ctype.ptr:
            raise NotImplementedError('fbinterface non-pointer object')
        return '&' + cxx2rust['fbinterface'][ctype.base]
    return build_rtype_string(ctype)


def build_rfunc_string(cfunc):
    this_type = Ctype(config["classname"], True, cfunc.const)
    args_str = f'this: {build_rtype_string(this_type)}, '
    for a in cfunc.args:
        args_str += f'{snakecase(a["name"])}: {build_rtype_string(a["type"])}, '
    args_str = args_str[:-2]
    ret_str = build_rtype_string(cfunc.rtype)
    if ret_str == cxx2rust["default"]["void"]:
        ret_str = ''
    else:
        ret_str = f' -> {ret_str}'
    return f'pub fn {prefix + "_" + snakecase(cfunc.name)}({args_str}){ret_str};'


def build_rfncall(cfunc):
    name_str = prefix + '_' + snakecase(cfunc.name)
    args_str = 'self.get_cthis(), ' if cfunc.const else 'self.get_this(), '
    for a in cfunc.args:
        args_str += f'{snakecase(a["name"])}'
        if a["type"].base in cxx2rust["fbinterface"]:
            args_str += '.this'
        args_str += ', '
    args_str = args_str[:-2]
    if get_ctype_category(cfunc.rtype) == 'fbinterface':
        return cxx2rust["fbinterface"][cfunc.rtype.base] + '{ this: ' + f'{name_str}({args_str})' + ' }'
    return f'{name_str}({args_str})'


def build_rfunc_trait_string(cfunc):
    name_str = snakecase(cfunc.name)
    args_str = '&self, '
    for a in cfunc.args:
        tp = a["type"]
        args_str += f'{snakecase(a["name"])}: {build_rtype_trait_string(tp)}, '
    args_str = args_str[:-2]
    ret_str = build_rtype_trait_string(cfunc.rtype)
    if ret_str == cxx2rust["default"]["void"]:
        ret_str = ''
    else:
        ret_str = f' -> {ret_str}'
    return 'fn %s(%s)%s\n{\n\tunsafe { return %s; }\n}' % (name_str, args_str, ret_str, build_rfncall(cfunc))


fc = open('config.json', 'r')
ft = open('target.cpp', 'r')
fr = open('result.txt', 'w')


config = json.load(fc)
prefix = snakecase(config['classname'][1:])
cxx2rust = config["cxx2rust"]

#123123

for _, v in cxx2rust["fbinterface"].items():
    print(v,'|',sep='',end='')
exit(0)

#123123

content = clear(ft.read())
lines = funclines(content)
funcs = [build_cfunc(l) for l in lines]
func_c, func_rust, func_rust_tr = '', '', ''
for cfunc in funcs:
    func_c += build_cfunc_string(cfunc) + '\n'
    rsfunc = build_rfunc_string(cfunc)
    func_rust += rsfunc + '\n'
    func_rust_tr += build_rfunc_trait_string(cfunc) + '\n'


fr.write(func_c + '\n\n\n\n')
fr.write(func_rust + '\n\n\n\n')
fr.write(func_rust_tr)


fr.close()
ft.close()
fc.close()