import { useEffect, useState } from "react";
import { SubmitHandler, useForm } from "react-hook-form";
import { FaAngleDown, FaAngleRight } from "react-icons/fa";

import { searchMovieList } from "@/api/movielist";
import TextBox from "@/components/form/TextBox";
import { DEFAULT_COUNT, DEFAULT_PAGE } from "@/constants";
import {
  SearchMovieListRequest,
  SearchMovieListResponse,
} from "@/schema/searchMovieList";

import ApiErrorMessage from "../ApiErrorMessage";
import DatePicker from "../form/DatePicker";
import TextArea from "../form/TextArea";
import style from "./style.module.css";

type SearchMovieListFormProps = {
  movieId: string;
  title: string;
  createdDate: string;
  description: string;
  userId: string;
  userName: string;
  // タグ検索を追加予定
};

type SearchMovieListProps = {
  page: number;
  count: number;
  onSearchResult: (data: SearchMovieListResponse) => void;
};

const SearchMovieList = ({
  page,
  count,
  onSearchResult,
}: SearchMovieListProps) => {
  const form = useForm<SearchMovieListFormProps>();
  const { handleSubmit } = form;
  const [IsAccordionOpen, setAccordionOpen] = useState<boolean>(false);
  const [error, setError] = useState<string>("");
  const handleClick = () => {
    setAccordionOpen(!IsAccordionOpen);
  };
  useEffect(() => {
    const fetchMovieList = async () => {
      const fetchMovieListRequest: SearchMovieListRequest = {
        page: DEFAULT_PAGE,
        count: DEFAULT_COUNT,
      };
      const res = await searchMovieList(fetchMovieListRequest);
      if ("message" in res) {
        setError(res.message);
      } else {
        setError("");
        onSearchResult(res);
      }
    };
    fetchMovieList();
  }, []);
  const onSubmit: SubmitHandler<SearchMovieListFormProps> = async (data) => {
    const searchMovieListRequest: SearchMovieListRequest = {
      movie_id: Number(data.movieId) || undefined,
      title: data.title || undefined,
      created_at: data.createdDate || undefined,
      description: data.description || undefined,
      user_id: Number(data.userId) || undefined,
      user_name: data.userName || undefined,
      page,
      count,
    };
    console.log("SearchMovieListRequest:", searchMovieListRequest);
    const res = await searchMovieList(searchMovieListRequest);
    if ("message" in res) {
      setError(res.message);
    } else {
      setError("");
      onSearchResult(res);
    }
  };

  return (
    <div className={style["search-movie-list-container"]}>
      <div className={style["search-movie-list-header"]} onClick={handleClick}>
        <span className={style["search-movie-list-header-title"]}>
          映画検索
        </span>
        {IsAccordionOpen ? (
          <div className={style["search-movie-list-header-icon-open"]}>
            <FaAngleDown />
          </div>
        ) : (
          <div className={style["search-movie-list-header-icon-close"]}>
            <FaAngleRight />
          </div>
        )}
      </div>
      <div
        className={style["search-movie-list-body"]}
        style={{ display: IsAccordionOpen ? "block" : "none" }}
      >
        <form
          onSubmit={handleSubmit(onSubmit)}
          className={style["search-movie-list-form"]}
        >
          <div className={style["search-movie-list-form-left-group"]}>
            <TextBox title="映画ID" name="movieId" form={form} />
            <DatePicker title="登録日" name="createdDate" form={form} />
            <TextBox title="タイトル" name="title" form={form} />
            <TextArea title="映画説明" name="description" form={form} />
          </div>
          <div className={style["search-movie-list-form-right-group"]}>
            <div className={style["register-name-group"]}>
              <label htmlFor="userId" className={style["register-name-label"]}>
                登録者
              </label>
              <div className={style["register-name-input-group"]}>
                <input
                  id="userId"
                  type="text"
                  placeholder="ユーザーID"
                  className={style["register-name-id"]}
                  {...form.register("userId")}
                />
                <input
                  id="userName"
                  type="text"
                  placeholder="登録者名"
                  className={style["register-name-input"]}
                  {...form.register("userName")}
                />
              </div>
            </div>
          </div>
          <button type="submit">検索</button>
          {/* TODO: 将来的にタグ検索を追加予定 */}
          <ApiErrorMessage errorMessage={error} />
        </form>
      </div>
    </div>
  );
};
export default SearchMovieList;
