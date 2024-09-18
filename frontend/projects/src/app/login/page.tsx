"use client"
import { yupResolver } from "@hookform/resolvers/yup";
import style from "./style.module.css";
import { useForm,SubmitHandler } from "react-hook-form";
import * as yup from "yup";
import TextBox from "@/components/form/TextBox";
import { useRouter } from "next/navigation";
import { useState } from "react";

export const Login = () =>{
    const router = useRouter();
    const [error,setError] = useState("");
    const loginSchema = yup.object({
        loginId: yup
            .string()
            .max(16,"16文字以内で入力してください．")
            .test("loginId-validation","アルファベット・数字・記号(_-@$)のみで入力してください．",function (value : any){
                return !!value.match(/^[0-9a-zA-Z_\-@\$]+$/);
            })
            .required("必須項目です．"),
        password: yup
            .string()
            .min(8,"8文字以上32文字以内で入力してください．")
            .max(32,"8文字以上32文字以内で入力してください．")
            .test("password-validation-available","アルファベット・数字・記号(_-@$)のみで入力してください．", function (value : any){
                return !!value.match(/^[0-9a-zA-Z_\-@\$]+$/);
            })
            .test("password-validation-required-character","アルファベットと数字はどちらも1文字以上含めてください．",function (value : any){
                return !!value.match(/^(?=.*[a-zA-Z])(?=.*[0-9])[_\-@\$a-zA-Z0-9]+$/)
            })
            .required("必須項目です．"),
    });
    type LoginForm = yup.InferType<typeof loginSchema>;
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
