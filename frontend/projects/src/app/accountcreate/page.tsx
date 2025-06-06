"use client";
import { yupResolver } from "@hookform/resolvers/yup";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { FormProvider, useForm } from "react-hook-form";

import { createAcount } from "@/api/account";
import ApiErrorMessage from "@/components/ApiErrorMessage";
import TextBox from "@/components/form/TextBox";
import ProvideErrorMessage from "@/components/ProvideErrorMessage";
import { HTTP_STATUS_OK } from "@/constants";
import {
  CreateAccountRequestBody,
  CreateAccountResponse,
} from "@/schema/createAccount";
import { LoginForm, loginSchema } from "@/schema/login";

import style from "./style.module.css";

export const AccountCreate = () => {
  const form = useForm<LoginForm>({
    resolver: yupResolver(loginSchema),
  });
  const router = useRouter();
  const [error, setError] = useState("");
  const { handleSubmit } = form;
  const handleSubmitAccountCreateButton = async (submitForm: LoginForm) => {
    const requestBody: CreateAccountRequestBody = {
      id: submitForm.loginId,
      password: submitForm.password,
    };
    const res: CreateAccountResponse = await createAcount(requestBody);
    // 200以外のステータスコードの場合はエラーメッセージを設定
    if (res.status !== HTTP_STATUS_OK) {
      setError(res.message);
      return;
    }
    router.push("../movielist");
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
          <ApiErrorMessage errorMessage={error} />
        </FormProvider>
      </form>
    </div>
  );
};

export default AccountCreate;
