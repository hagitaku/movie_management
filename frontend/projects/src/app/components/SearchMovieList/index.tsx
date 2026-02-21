import { useState } from "react";
import { SubmitHandler, useForm } from "react-hook-form";
import { FaAngleDown, FaAngleRight } from "react-icons/fa";

import TextBox from "@/components/form/TextBox";

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
const SearchMovieList = () => {
  const form = useForm<SearchMovieListFormProps>();
  const { handleSubmit } = form;
  const [IsAccordionOpen, setAccordionOpen] = useState<boolean>(false);
  const handleClick = () => {
    setAccordionOpen(!IsAccordionOpen);
  };
  const onSubmit: SubmitHandler<SearchMovieListFormProps> = (data) =>
    console.log(data);

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
          <TextBox title="映画ID" name="movieId" form={form} />
          <TextBox title="登録者ID" name="userId" form={form} />
          <TextBox title="タイトル" name="title" form={form} />
          <TextBox title="登録者名" name="userName" form={form} />
          <DatePicker title="登録日" name="createdDate" form={form} />
          <TextArea title="映画説明" name="description" form={form} />
          {/* TODO: 将来的にタグ検索を追加予定 */}
          <button type="submit">検索</button>
        </form>
      </div>
    </div>
  );
};
export default SearchMovieList;
