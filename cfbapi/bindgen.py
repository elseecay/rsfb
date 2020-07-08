import json
import re
from collections import namedtuple
from stringcase import snakecase


def remove_template(content):
    r = re.compile(r"template[\s]*<[\w\s]+>[\s]*")
    return r.sub("", content)


def remove_function_body(content):
    return content
    #r = re.compile(r"{((\s)|(.))*}")
    #return r.sub("", content)


def remove_semicolon(content):
    return content.replace(';', '')


def only_spaces(content):
    r = re.compile(r"\s")
    return r.sub(" ", content)


def remove_mult_spaces(content):
    r = re.compile(r" {2,}")
    return r.sub(" ", content)


def funclines(content):
    return [(s + ')').strip() for s in content.split(')')[:-1]]


def build_funcdesc(line):
    funcdesc = namedtuple('funcdesc', ('name', 'rtype', 'args'))

    r = re.compile(r'\w+\(')
    m = r.search(line)
    funcdesc.name = line[m.start():m.end() - 1]
    funcdesc.rtype = line[:m.start()].strip()
    line = line[m.end():-1]

    funcdesc.args = []
    r = re.compile(r'\w+$')
    splist = line.split(',')
    if len(splist) == 1 and not len(splist[0]):
        return funcdesc
    for arg in line.split(','):
        arg = arg.strip()
        m = r.search(arg)
        funcdesc.args.append({"type": arg[:m.start() - 1], "name": arg[m.start():m.end()]})
    return funcdesc


def is_std_type(t):
    return t in config['cxx2rust']['type']


def is_fb_type(t):
    return t in config['cxx2rust']['fbinterface']


def check_unknown_type(t):
    if not is_fb_type(t) and not is_std_type(t):
        raise Exception(f'Unknown type {t}')


def get_rust_typename(t):
    if is_fb_type(t):
        return config['cxx2rust']['fbclass'][t]
    if is_std_type(t):
        return config['cxx2rust']['type'][t]


def build_rust_typename(cpptype, ptr=None, const=None):
    rstype = get_rust_typename(cpptype)
    if ptr:
        if const:
            return f'CPtr<{rstype}>' # std type
        if is_std_type(cpptype):
            return f'Ptr<{rstype}>'
        else:
            return f'{rstype}Ptr'
    return rstype


def get_cpptype_info(t):
    if t.count('*') > 1:
        raise Exception(f'{t.count("*")}x pointer')
    if t.count('('):
        raise Exception('Function pointer')
    if t.find('const') > t.find('*'):
        raise Exception('Const on top level')
    ptr, const = False, False
    if t.count('*'):
        t = t.replace('*', '')
        ptr = True
        if t.count('const'):
            t = t.replace('const', '')
            const = True
        t = t.strip()
    check_unknown_type(t)
    return t, ptr, const


def get_rust_type(t):
    return build_rust_typename(*get_cpptype_info(t))


def get_rust_function(fd):
    s = f'pub fn {prefix + "_" + snakecase(fd.name)}(this: {config["cxx2rust"]["fbinterface"][config["classname"]]}Ptr, '
    for a in fd.args:
        s += f'{snakecase(a["name"])}: {get_rust_type(a["type"])}, '
    s = s[:-2]
    s += ')'
    if fd.rtype != 'void':
        s += f' -> {get_rust_type(fd.rtype)}'
    s += ';'
    return s


def get_rust_fncall(fd):
    call = prefix + '_' + snakecase(fd.name) + '(self.get_this()'
    for a in fd.args:
        call += f', {snakecase(a["name"])}'
        if is_fb_type(get_cpptype_info(a["type"])[0]):
            call += '.this'
    call += ')'
    return call


def get_rust_trait_function(rsfunc, fd):
    r = re.compile(r'pub\s*fn')
    rsfunc = r.sub('fn', rsfunc)
    rsfunc = rsfunc[:-1]
    rsfunc = rsfunc.replace(prefix + '_', '')
    r = re.compile(r'this:\s*\w+')
    rsfunc = r.sub('&self', rsfunc)
    r = re.compile(r'(\w+)Ptr([,)])')
    rsfunc = r.sub(r'&\1\2', rsfunc)
    r = re.compile(r'->\s*(\w+)Ptr')
    rsfunc = r.sub(r'-> \1', rsfunc)
    rsfunc += '\n{\n\tunsafe { return '
    base_ret_type = get_cpptype_info(fd.rtype)[0]
    if is_fb_type(base_ret_type):
        rsfunc += config["cxx2rust"]["fbinterface"][base_ret_type] + '{ this: ' + get_rust_fncall(fd) + ' }'
    else:
        rsfunc += get_rust_fncall(fd)
    rsfunc += '; }\n}'
    return rsfunc


def get_externc_function(fd):
    result = f'{fd.rtype} {prefix + "_" + snakecase(fd.name)}({config["classname"]}* self'
    for a in fd.args:
        result += f', {a["type"]} {a["name"]}'
    result += ')\n{\n\t'
    result += f'self->{fd.name}('
    for a in fd.args:
        result += f'{a["name"]}, '
    if len(fd.args):
        result = result[:-2]
    result += ');\n}'
    return result


fc = open('config.json', 'r')
ft = open('target.cpp', 'r')
fr = open('result.txt', 'w')

config = json.load(fc)
prefix = config['prefix']
content = ft.read()

content = only_spaces(content)
content = remove_mult_spaces(content)
content = remove_function_body(content)
content = remove_semicolon(content)
content = remove_template(content)
lines = funclines(content)
fds = [build_funcdesc(l) for l in lines]

func_c, func_rust, func_rust_tr = '', '', ''
for fd in fds:
    func_c += get_externc_function(fd) + '\n'
    rsfunc = get_rust_function(fd)
    func_rust += rsfunc + '\n'
    func_rust_tr += get_rust_trait_function(rsfunc, fd) + '\n'


fr.write(func_c + '\n\n\n\n')
fr.write(func_rust + '\n\n\n\n')
fr.write(func_rust_tr)


fr.close()
ft.close()
fc.close()