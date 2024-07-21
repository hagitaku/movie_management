import { BASE_API_URL, REGISTER_POST_PATH } from "@/constants/constants";
import axios from "axios";

type PostCreateMovieRequest = {
  title: string;
  description: string;
  memo: string;
  session_token: string;
};
const postCreate = async (request: PostCreateMovieRequest) => {
  const postUrl = BASE_API_URL + REGISTER_POST_PATH;
  try {
    const response = await axios.post(postUrl, request);
    return response;
  } catch (error) {
    throw error;
  }
};
export default postCreate;
