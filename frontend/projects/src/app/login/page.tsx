"use client";
import { yupResolver } from "@hookform/resolvers/yup";
import style from "./style.module.css";
import { useForm, SubmitHandler, FormProvider } from "react-hook-form";
import { loginSchema, LoginForm } from "@/schema/login";
import TextBox from "@/components/form/TextBox";
import { useRouter } from "next/navigation";
import { useState } from "react";
import ProvideErrorMessage from "@/components/ProvideErrorMessage";
import ApiErrorMessage from "@/components/ApiErrorMessage";

export const Login = () => {
  const router = useRouter();
  const [error, setError] = useState("");
  const form = useForm<LoginForm>({
    resolver: yupResolver(loginSchema),
  });
  const { handleSubmit } = form;
  const handleClickLoginButton: SubmitHandler<LoginForm> = async (
    submitForm: LoginForm
  ) => {
    try {
      // TODO: ユーザログインのAPIが完成次第，処理を追加
      const response = "success";
      const status = 200;
      if (status == 200) {
        router.push("../movielist");
      } else if (status == 400) {
        setError("ユーザ名またはパスワードが間違っています．");
      } else {
        setError("ログインに失敗しました．もう一度お試しください．");
      }
    } catch (error) {
      // エラーハンドリング
      setError("サーバーエラーが発生しました．");
    }
  };

  return (
    <div className={style["login-form-container"]}>
      <FormProvider {...form}>
        <form
          className={style["login-form"]}
          onSubmit={handleSubmit(handleClickLoginButton)}
        >
          <div className={style["login-id-field"]}>
            <TextBox title="ID" name="loginId" form={form} />
            <ProvideErrorMessage<LoginForm> name="loginId" />
          </div>
          <div className={style["login-password-field"]}>
            <TextBox
              title="パスワード"
              name="password"
              type="password"
              form={form}
            />
            <ProvideErrorMessage<LoginForm> name="password" />
          </div>
          <div className={style["login-submit"]}>
            <button className={style["login-submit-button"]} type="submit">
              登録
            </button>
          </div>
          <ApiErrorMessage errorMessage={error} />
        </form>
      </FormProvider>
    </div>
  );
};
export default Login;
