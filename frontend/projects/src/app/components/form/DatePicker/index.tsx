import type { FieldValues, Path } from "react-hook-form";
import { UseFormReturn } from "react-hook-form";

import style from "./style.module.css";

type DatePicker<T extends FieldValues> = {
  title: string;
  name: Path<T>;
  form: UseFormReturn<T>;
};
const DatePicker = <T extends FieldValues>(props: DatePicker<T>) => {
  const { title, name, form } = props;
  const { register } = form;

  return (
    <div className={style["date-picker-field"]}>
      <label htmlFor={name}>{title}</label>
      <div className={style["date-picker-input"]}>
        <input id={name} {...register(name)} type="date"></input>
      </div>
    </div>
  );
};
export default DatePicker;
