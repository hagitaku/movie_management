"use client";
import { useState } from "react";

import SearchMovieList from "@/components/SearchMovieList";
import Pagination from "@/components/ui/Pagination";
import ViewMovieList from "@/components/ViewMovieList";
import { DEFAULT_COUNT, DEFAULT_PAGE } from "@/constants";
import { SearchMovieListResponse } from "@/schema/searchMovieList";

import style from "./style.module.css";

export const MovieList = () => {
  const [page, setPage] = useState<number>(DEFAULT_PAGE);
  const [, setTotalCount] = useState<number>(0);
  const [, setTotalPage] = useState<number>(DEFAULT_PAGE);

  const [viewMovieList, setViewMovieList] = useState<SearchMovieListResponse>({
    movie_list: [],
    total_count: 0,
    page: DEFAULT_PAGE,
    count: DEFAULT_COUNT,
  });
  // 検索結果を受け取るコールバック関数
  const handleSearchResult = (data: SearchMovieListResponse) => {
    setViewMovieList(data);
    setTotalCount(data.total_count);
    setTotalPage(Math.ceil(data.total_count / data.count));
  };
  return (
    <div className={style["movie-list-container"]}>
      <Pagination currentPage={page} totalPage={5} setCurrentPage={setPage} />
      <SearchMovieList
        page={DEFAULT_PAGE}
        count={DEFAULT_COUNT}
        onSearchResult={handleSearchResult}
      />
      <ViewMovieList viewMovieList={viewMovieList} />
      <Pagination currentPage={page} totalPage={5} setCurrentPage={setPage} />
    </div>
  );
};
export default MovieList;
