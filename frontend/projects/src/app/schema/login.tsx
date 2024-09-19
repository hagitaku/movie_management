import * as yup from "yup";
export const loginSchema = yup.object({
    loginId: yup
        .string()
        .required("必須項目です．")
        .max(16,"16文字以内で入力してください．")
        .test("loginId-validation","アルファベット・数字・記号(_-@$)のみで入力してください．",function (value : any){
            return !!value.match(/^[0-9a-zA-Z_\-@\$]+$/);
        }),
    password: yup
        .string()
        .required("必須項目です．")
        .min(8,"8文字以上32文字以内で入力してください．")
        .max(32,"8文字以上32文字以内で入力してください．")
        .test("password-validation-available","アルファベット・数字・記号(_-@$)のみで入力してください．", function (value : string){
            return !!value.match(/^[0-9a-zA-Z_\-@\$]+$/);
        })
        .test("password-validation-required-character","アルファベットと数字はどちらも1文字以上含めてください．",function (value : string){
            return !!value.match(/^(?=.*[a-zA-Z])(?=.*[0-9])[_\-@\$a-zA-Z0-9]+$/)
        }),
});
export type LoginForm = yup.InferType<typeof loginSchema>;