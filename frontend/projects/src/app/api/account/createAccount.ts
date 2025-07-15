import axios from "axios";

import {
  BASE_API_URL,
  CREATE_ACCOUNT_PATH,
  HTTP_MESSAGE_UNKNOWN,
} from "@/constants";
import {
  CreateAccountRequestBody,
  CreateAccountResponse,
  CreateAccountResponseBody,
} from "@/schema/createAccount";

export const createAccount = async (req: CreateAccountRequestBody) => {
  const url = new URL(`${BASE_API_URL}${CREATE_ACCOUNT_PATH}`);
  try {
    const res = await axios.post<CreateAccountResponseBody>(
      url.toString(),
      req,
    );
    // ステータスコードに応じてレスポンスメッセージを設定
    const createAccountResponse: CreateAccountResponse = {
      message: res.data.message,
    };
    return createAccountResponse;
  } catch (error) {
    // unknownでないことを確認してハンドリング
    if (axios.isAxiosError(error)) {
      const createAccountResponse: CreateAccountResponse = {
        message: error.response?.data.message || HTTP_MESSAGE_UNKNOWN,
      };
      return createAccountResponse;
    } else {
      // こっちに来る場合は500エラーとして処理
      const createAccountResponse: CreateAccountResponse = {
        message: HTTP_MESSAGE_UNKNOWN,
      };
      return createAccountResponse;
    }
  }
};
