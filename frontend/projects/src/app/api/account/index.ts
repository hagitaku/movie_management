import axios from "axios";

import { BASE_API_URL } from "@/constants";
import {
  CreateAccountRequestBody,
  CreateAccountResponse,
  CreateAccountResponseBody,
} from "@/schema/createAccount";

export const CreateAcount = async (req: CreateAccountRequestBody) => {
  const url = new URL(`${BASE_API_URL}account/register`);
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
    throw new Error(
      `Failed to create account: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
};
