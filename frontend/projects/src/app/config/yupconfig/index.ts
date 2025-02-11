import * as yup from "yup";

export type MessageParams = {
  path: string;
  value: any;
  originalValue: any;
  label: string;
  type: string;
};

const labelText = (prm: MessageParams) => {
  return prm.label !== "" ? `${prm.label}は` : "";
};

// エラーメッセージの日本語表記
const jpConfig = {
  mixed: {
    default: (prm: MessageParams) => `${labelText(prm)}無効です`,
    required: (prm: MessageParams) => `${labelText(prm)}必須の入力項目です`,
    oneOf: (prm: MessageParams & { values: any }) =>
      `${labelText(prm)}次の値のいずれかを入力してください:${prm.values}`,
    notOneOf: (prm: MessageParams & { values: any }) =>
      `${labelText(prm)}次の値のいずれかを入力してください:${prm.values}`,
    notType: `形式が違います`,
    defined: ``,
  },
  string: {
    length: (prm: MessageParams & { length: number }) =>
      `${labelText(prm)}${prm.length}文字で入力してください`,
    min: (prm: MessageParams & { min: number }) =>
      `${labelText(prm)}少なくとも${prm.min}文字で入力してください`,
    max: (prm: MessageParams & { max: number }) =>
      `${labelText(prm)}最大${prm.max}文字で入力してください`,
    matches: (prm: MessageParams & { regex: RegExp }) =>
      `${labelText(prm)}次の形式と一致する必要があります: "${prm.regex}"`,
    email: (prm: MessageParams & { regex: RegExp }) =>
      `${labelText(prm)}メールアドレス形式で入力してください`,
    onlyAlphabetNumberSymbol: (prm: MessageParams) =>
      `${labelText(prm)}アルファベット・数字・記号(_-@$)のみで入力してください`,
    requireAlphabetNumber: (prm: MessageParams) =>
      `${labelText(prm)}アルファベットと数字はどちらも1文字以上含めてください`,
  },
};

yup.setLocale(jpConfig);
type StringValidationType = {
  name: string;
  errorMessage: (prm: MessageParams) => string;
  isValid: (value: string) => boolean;
};

const stringValidationList: StringValidationType[] = [
  {
    name: "onlyAlphabetNumberSymbol",
    errorMessage: jpConfig.string.onlyAlphabetNumberSymbol,
    isValid: (value: string) => {
      return !!value.match(/^[0-9a-zA-Z_\-@\$]+$/);
    },
  },
  {
    name: "requireAlphabetNumber",
    errorMessage: jpConfig.string.requireAlphabetNumber,
    isValid: (value: string) => {
      return !!value.match(/^(?=.*[a-zA-Z])(?=.*[0-9])[_\-@\$a-zA-Z0-9]+$/);
    },
  },
];

stringValidationList.forEach((validation) => {
  yup.addMethod<yup.StringSchema>(
    yup.string,
    validation.name,
    function (message: yup.Message = validation.errorMessage) {
      return this.test(function (value, testContext) {
        if (value == null || value === "") {
          return true;
        }
        if (validation.isValid(value)) {
          return true;
        }
        return testContext.createError({
          message,
        });
      });
    }
  );
});
declare module "yup" {
  interface StringSchema<TType, TContext, TDefault, TFlags> {
    onlyAlphabetNumberSymbol(): StringSchema<TType, TContext, TDefault, TFlags>;
    requireAlphabetNumber(): StringSchema<TType, TContext, TDefault, TFlags>;
  }
}
export default yup;
