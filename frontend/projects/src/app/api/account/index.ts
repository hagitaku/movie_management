import axios from "axios";

import {
  BASE_API_URL,
  CREATE_ACCOUNT_PATH,
  HTTP_STATUS_INTERNAL_SERVER_ERROR,
} from "@/constants";
import {
  CreateAccountRequestBody,
  CreateAccountResponse,
  CreateAccountResponseBody,
} from "@/schema/createAccount";

export const CreateAcount = async (req: CreateAccountRequestBody) => {
  const url = new URL(`${BASE_API_URL}${CREATE_ACCOUNT_PATH}`);
  try {
    const res = await axios.post<CreateAccountResponseBody>(
      url.toString(),
      req,
    );
    // ステータスコードに応じてレスポンスメッセージを設定
    const createAccountResponse: CreateAccountResponse = {
      message: res.data.message,
      status: res.status,
    };
    return createAccountResponse;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      const createAccountResponse: CreateAccountResponse = {
        message: error.response?.data.message || "Unknown error",
        status: error.response?.status || HTTP_STATUS_INTERNAL_SERVER_ERROR,
      };
      return createAccountResponse;
    } else {
      // Axios以外のエラーの場合は、500エラーとして処理
      const createAccountResponse: CreateAccountResponse = {
        message: "Unknown error",
        status: HTTP_STATUS_INTERNAL_SERVER_ERROR,
      };
      return createAccountResponse;
    }
  }
};
