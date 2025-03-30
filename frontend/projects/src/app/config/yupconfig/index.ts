import * as yup from "yup";

export type MessageParams = {
  path: string;
  label: string;
  length: number;
};

const labelText = (label: string) => {
  return label ? `${label}は` : "";
};

// エラーメッセージの日本語表記
const jpConfig: yup.LocaleObject = {
  mixed: {
    default: ({ label }) => `${labelText(label)}無効です`,
    required: ({ label }) => `${labelText(label)}必須の入力項目です`,
    oneOf: ({ label, values }) =>
      `${labelText(label)}次の値のいずれかを入力してください:${values}`,
    notOneOf: ({ label, values }) =>
      `${labelText(label)}次の値のいずれかを入力してください:${values}`,
    notType: `形式が違います`,
    defined: ``,
  },
  string: {
    length: ({ label, length }) =>
      `${labelText(label)}${length}文字で入力してください`,
    min: ({ label, min }) =>
      `${labelText(label)}少なくとも${min}文字で入力してください`,
    max: ({ label, max }) =>
      `${labelText(label)}最大${max}文字で入力してください`,
    matches: ({ label, regex }) =>
      `${labelText(label)}次の形式と一致する必要があります: "${regex}"`,
    email: ({ label }) =>
      `${labelText(label)}メールアドレス形式で入力してください`,
  },
};

yup.setLocale(jpConfig);

yup.addMethod<yup.StringSchema>(
  yup.string,
  "onlyAlphabetNumberSymbol",
  function (message?: string) {
    return this.matches(new RegExp(/^[0-9a-zA-Z\_\-@$]+$/), {
      message:
        message || "アルファベット・数字・記号(_-@$)のみで入力してください",
    });
  }
);

yup.addMethod<yup.StringSchema>(
  yup.string,
  "requireAlphabetNumber",
  function (message?: string) {
    return this.matches(
      new RegExp(/^(?=.*[a-zA-Z])(?=.*[0-9])[\_\-@$a-zA-Z0-9]+$/),
      {
        message:
          message || "アルファベットと数字はどちらも1文字以上含めてください",
      }
    );
  }
);

declare module "yup" {
  interface StringSchema<TType, TContext, TDefault, TFlags> {
    onlyAlphabetNumberSymbol(): StringSchema<TType, TContext, TDefault, TFlags>;
    requireAlphabetNumber(): StringSchema<TType, TContext, TDefault, TFlags>;
  }
}
export default yup;
