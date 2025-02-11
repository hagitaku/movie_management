import yup from "@/config/yupconfig";

export const loginSchema = yup.object({
  loginId: yup
    .string()
    .required()
    .label("ログインID")
    .max(16)
    .onlyAlphabetNumberSymbol(),
  password: yup
    .string()
    .required()
    .label("パスワード")
    .min(8)
    .max(32)
    .onlyAlphabetNumberSymbol()
    .requireAlphabetNumber(),
});
export type LoginForm = yup.InferType<typeof loginSchema>;
