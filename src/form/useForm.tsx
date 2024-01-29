import { useState, ChangeEvent } from 'react';

type FormValues = { [key: string]: any };

const useForm = (initialValues: FormValues) => {
  const [values, setValues] = useState<FormValues>(initialValues);
  
  const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
    const { name, value } = event.target;
    setValues({ ...values, [name]: value });
  };

  return [values, handleChange] as const;
};

export default useForm;
