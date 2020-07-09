void clearException()
		{
			if (dirty)
			{
				dirty = false;
				status->init();
			}
		}



virtual void dispose()
		{
			// Disposes only the delegated status. Let the user destroy this instance.
			status->dispose();
		}

		virtual void init()
		{
			clearException();
		}

		virtual unsigned getState() const
		{
			return dirty ? status->getState() : 0;
		}

		virtual void setErrors2(unsigned length, const intptr_t* value)
		{
			dirty = true;
			status->setErrors2(length, value);
		}

		virtual void setWarnings2(unsigned length, const intptr_t* value)
		{
			dirty = true;
			status->setWarnings2(length, value);
		}

		virtual void setErrors(const intptr_t* value)
		{
			dirty = true;
			status->setErrors(value);
		}

		virtual void setWarnings(const intptr_t* value)
		{
			dirty = true;
			status->setWarnings(value);
		}

		virtual const intptr_t* getErrors() const
		{
			return dirty ? status->getErrors() : cleanStatus();
		}

		virtual const intptr_t* getWarnings() const
		{
			return dirty ? status->getWarnings() : cleanStatus();
		}

		virtual IStatus* clone() const
		{
			return status->clone();
		} 
