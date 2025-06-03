export interface CreateAccountRequestBody {
  id: string;
  password: string;
}
export interface CreateAccountResponse {
  message: string;
  status: number;
}
export interface CreateAccountResponseBody {
  message: string;
}
