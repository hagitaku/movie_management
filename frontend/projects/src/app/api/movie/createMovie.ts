import axios from "axios";

import { BASE_API_URL, REGISTER_POST_PATH } from "@/constants";

type PostCreateMovieRequest = {
  title: string;
  description: string;
  memo: string;
  session_token: string;
};
export const postCreate = async (request: PostCreateMovieRequest) => {
  const postUrl = BASE_API_URL + REGISTER_POST_PATH;
  // TODO: エラーハンドリングを追加する
  //try {
  const response = await axios.post(postUrl, request);
  return response;
  //} catch (error) {
  //throw error;
  //}
};
