import { FieldValues, useFormContext } from "react-hook-form";
import style from "./style.module.css";

type ErrorMessageProps<T extends FieldValues> = {
  name: keyof T;
};
const ProvideErrorMessage = <T extends FieldValues>({
  name,
}: ErrorMessageProps<T>) => {
  const {
    formState: { errors },
  } = useFormContext<T>();
  const error = errors[name];
  const errorMessage =
    error && "message" in error && typeof error.message == "string"
      ? error.message
      : undefined; // 型ガードとプロパティ判定

  return (
    errorMessage && <div className={style["error-message"]}>{errorMessage}</div>
  );
};

export default ProvideErrorMessage;
