import axios from "axios";

import {
  BASE_API_URL,
  HTTP_MESSAGE_UNKNOWN,
  SEARCH_MOVIE_LIST_PATH,
} from "@/constants";
import {
  SearchMovieListErrorResponse,
  SearchMovieListRequest,
  SearchMovieListResponse,
} from "@/schema/searchMovieList";

export const searchMovieList = async (req: SearchMovieListRequest) => {
  const url = new URL(`${BASE_API_URL}${SEARCH_MOVIE_LIST_PATH}`);
  try {
    const res = await axios.post<SearchMovieListResponse>(url.toString(), req);
    const searchMovieListResponse: SearchMovieListResponse = {
      movie_list: res.data.movie_list,
      total_count: res.data.total_count,
      page: res.data.page,
      count: res.data.count,
    };
    return searchMovieListResponse;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      const searchMovieListErrorResponse: SearchMovieListErrorResponse = {
        message: error.response?.data.message || HTTP_MESSAGE_UNKNOWN,
      };
      return searchMovieListErrorResponse;
    } else {
      // 500エラーとして処理
      const searchMovieListErrorResponse: SearchMovieListErrorResponse = {
        message: HTTP_MESSAGE_UNKNOWN,
      };
      return searchMovieListErrorResponse;
    }
  }
};
