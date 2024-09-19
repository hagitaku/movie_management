"use client"
import { yupResolver } from "@hookform/resolvers/yup";
import style from "./style.module.css";
import { useForm,SubmitHandler } from "react-hook-form";
import {loginSchema,LoginForm} from "@/schema/login";
import TextBox from "@/components/form/TextBox";
import { useRouter } from "next/navigation";
import { useState } from "react";

export const Login = () =>{
    const router = useRouter();
    const [error,setError] = useState("");   
    const form = useForm<LoginForm>({
        resolver: yupResolver(loginSchema)
    });
    const {
        handleSubmit,
        formState: {errors}
    } = form;
    const handleClickLoginButton : SubmitHandler<LoginForm> = (submitForm : LoginForm) => {
        try{
            // TODO: ユーザログインのAPIが完成次第，処理を追加
            const response = "success";
            const status = 200;
            if(status == 200){
                router.push("../movielist")
                console.log(submitForm);
                alert("ログインしました！");
            }
            else if(status == 400){
                setError("ユーザ名またはパスワードが間違っています．");
            }
            else{
                setError("ログインに失敗しました．もう一度お試しください．");
            }
        }catch(error){
            // エラーハンドリング
            setError("サーバーエラーが発生しました．");
        }
    }

    return (
        <div className={style["login-form-container"]}>
            <form className={style["login-form"]} onSubmit={handleSubmit(handleClickLoginButton)}>
                <div className={style["login-id-field"]}>
                    <TextBox title="ID" name="loginId" form={form} />
                    <div className={style["login-id-error-message"]}>
                        {errors.loginId && errors.loginId.message}
                    </div>
                </div>
                <div className={style["login-password-field"]}>
                    <TextBox title="パスワード" name="password" type="password" form={form} />
                    <div className={style["login-password-error-message"]}>
                        {errors.password && errors.password.message}
                    </div>
                </div>
                <div className={style["login-submit"]}>
                    <button className={style["login-submit-button"]} type="submit">
                        登録
                    </button>
                </div>
            </form>
        </div>
    )
}
export default Login;
