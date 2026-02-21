export enum SortKey {
  MOVIE_ID = "movieId",
  TITLE = "title",
  USER_NAME = "userName",
  DATE = "date",
}
export interface SearchMovieListRequest {
  movie_id?: string;
  title?: string;
  created_at?: string;
  description?: string;
  user_id?: string;
  user_name?: string;
  sort_key?: SortKey;
  page: number;
  count: number;
}

export interface SearchMovieResponse {
  movie_id: string;
  title: string;
  description: string;
  user_name: string;
  created_date: string;
  user_id: string;
}

export interface SearchMovieListResponse {
  movie_list: SearchMovieResponse[];
  total_count: number;
  page: number;
  count: number;
}

export interface SearchMovieListErrorResponse {
  message: string;
}
