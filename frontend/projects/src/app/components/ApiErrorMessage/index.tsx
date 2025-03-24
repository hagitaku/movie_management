import style from "./style.module.css";
type ApiErrorMessageProps = {
  errorMessage: string;
};
const ApiErrorMessage = ({ errorMessage }: ApiErrorMessageProps) => {
  return (
    errorMessage && (
      <div className={style["error-message"]}>
        <div>{errorMessage}</div>
      </div>
    )
  );
};
export default ApiErrorMessage;
