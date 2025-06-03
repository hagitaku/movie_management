"use client";
import { yupResolver } from "@hookform/resolvers/yup";
import { useRouter } from "next/navigation";
import { FormProvider, useForm } from "react-hook-form";

import TextBox from "@/components/form/TextBox";
import ProvideErrorMessage from "@/components/ProvideErrorMessage";
import { LoginForm, loginSchema } from "@/schema/login";

import style from "./style.module.css";

export const AccountCreate = () => {
  const form = useForm<LoginForm>({
    resolver: yupResolver(loginSchema),
  });
  const rounter = useRouter();
  const { handleSubmit } = form;
  const handleSubmitAccountCreateButton = async (submitForm: LoginForm) => {
    console.log(submitForm);
    rounter.push("../movielist");
  };
  return (
    <div className={style["account-create-form-container"]}>
      <form
        className={style["account-create-form"]}
        onSubmit={handleSubmit(handleSubmitAccountCreateButton)}
      >
        <FormProvider {...form}>
          <div className={style["account-create-id-field"]}>
            <TextBox
              title="ログインID"
              name="loginId"
              type="text"
              form={form}
            />
            <ProvideErrorMessage<LoginForm> name="loginId" />
          </div>
          <div className={style["account-create-password-field"]}>
            <TextBox
              title="パスワード"
              name="password"
              type="password"
              form={form}
            />
          </div>
          <div className={style["account-create-submit"]}>
            <button
              className={style["account-create-submit-button"]}
              type="submit"
            >
              アカウント作成
            </button>
            <ProvideErrorMessage<LoginForm> name="password" />
          </div>
        </FormProvider>
      </form>
    </div>
  );
};

export default AccountCreate;
