"use client";
import { useState } from "react";

import { searchMovieList } from "@/api/movielist";
import SearchMovieList from "@/components/SearchMovieList";
import Pagination from "@/components/ui/Pagination";
import ViewMovieList from "@/components/ViewMovieList";
import { DEFAULT_COUNT, DEFAULT_PAGE, MAX_PAGINATION_ITEM } from "@/constants";
import {
  SearchMovieListRequest,
  SearchMovieListResponse,
} from "@/schema/searchMovieList";

import style from "./style.module.css";

export const MovieList = () => {
  const [page, setPage] = useState<number>(DEFAULT_PAGE);
  const [count] = useState<number>(DEFAULT_COUNT);
  const [totalCount, setTotalCount] = useState<number>(0);
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

  const handleClickPage = async (page: number) => {
    setPage(page);
    const movieListRequestWithCurrentPageAndCount: SearchMovieListRequest = {
      page,
      count,
    };
    const response = await searchMovieList(
      movieListRequestWithCurrentPageAndCount,
    );
    if ("message" in response) {
      // TODO: エラーハンドリング
    } else {
      handleSearchResult(response);
    }
  };
  return (
    <div className={style["movie-list-container"]}>
      <Pagination
        currentPage={page}
        totalPage={totalCount}
        handleClickPage={handleClickPage}
        maxPaginationItem={MAX_PAGINATION_ITEM}
      />
      <SearchMovieList
        page={DEFAULT_PAGE}
        count={DEFAULT_COUNT}
        onSearchResult={handleSearchResult}
      />
      <ViewMovieList viewMovieList={viewMovieList} />
      <Pagination
        currentPage={page}
        totalPage={totalCount}
        handleClickPage={handleClickPage}
        maxPaginationItem={MAX_PAGINATION_ITEM}
      />
    </div>
  );
};
export default MovieList;
