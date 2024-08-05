"use client"
import { yupResolver } from "@hookform/resolvers/yup";
import style from "./style.module.css";
import { useForm,SubmitHandler } from "react-hook-form";
import * as yup from "yup";
import TextBox from "@/components/form/TextBox";

export const Login = () =>{
    const loginSchema = yup.object({
        loginId: yup
            .string()
            .max(10,"10文字以内で入力してください．")
            .test("only-alphabet","アルファベットのみで入力してください．",function (value : any){
                return !!value.match(/^[a-zA-Z]+$/);
            })
            .required("必須項目です．"),
        password: yup
            .string()
            .min(8,"8文字以上20文字以内で入力してください．")
            .max(20,"8文字以上20文字以内で入力してください．")
            .test("no-specific-character","アルファベット・数字・記号(@/*)のみで入力してください．", function (value : any){
                return !!value.match(/^[0-9a-zA-Z@\/\*]+$/);
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
        console.log(submitForm);
        alert("ログインしました！");
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
