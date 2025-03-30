import yup from "@/config/yupconfig";
import {
  MAX_LOGIN_ID_LENGTH,
  MAX_LOGIN_PASSWORD_LENGTH,
  MIN_LOGIN_PASSWORD_LENGTH,
} from "@/constants";

export const loginSchema = yup.object({
  loginId: yup
    .string()
    .required()
    .label("ログインID")
    .max(MAX_LOGIN_ID_LENGTH)
    .onlyAlphabetNumberSymbol(),
  password: yup
    .string()
    .required()
    .label("パスワード")
    .min(MIN_LOGIN_PASSWORD_LENGTH)
    .max(MAX_LOGIN_PASSWORD_LENGTH)
    .onlyAlphabetNumberSymbol()
    .requireAlphabetNumber(),
});
export type LoginForm = yup.InferType<typeof loginSchema>;
