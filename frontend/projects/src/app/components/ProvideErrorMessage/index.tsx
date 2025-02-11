import { useFormContext } from "react-hook-form";
import style from "./style.module.css";

type ErrorMessageProps = {
  name: string;
};
const ProvideErrorMessage = ({ name }: ErrorMessageProps) => {
  const {
    formState: { errors },
  } = useFormContext();
  const errorMessage = errors[name]?.message as string | undefined;

  return errorMessage ? (
    <div className={style["error-message"]}>{errorMessage}</div>
  ) : null;
};

export default ProvideErrorMessage;
