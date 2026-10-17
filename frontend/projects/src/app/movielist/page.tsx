"use client";
import { useEffect, useState } from "react";

import { searchMovieList } from "@/api/movielist";
import ApiErrorMessage from "@/components/ApiErrorMessage";
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
  const [error, setError] = useState<string>("");

  const [viewMovieList, setViewMovieList] = useState<SearchMovieListResponse>({
    movie_list: [],
    total_count: 0,
    page: DEFAULT_PAGE,
    count: DEFAULT_COUNT,
  });
  const handleClickPage = async (page: number) => {
    setPage(page);
  };

  const getMovieListWith = async (request: SearchMovieListRequest) => {
    const response = await searchMovieList(request);
    if ("message" in response) {
      setError(response.message);
    } else {
      setError("");
      setViewMovieList(response);
      setTotalCount(response.total_count);
      setTotalPage(Math.ceil(response.total_count / response.count));
    }
  };
  useEffect(() => {
    const fetchMovieListRequest: SearchMovieListRequest = {
      page,
      count,
    };
    getMovieListWith(fetchMovieListRequest);
  }, [page, count]);
  return (
    <div className={style["movie-list-container"]}>
      <Pagination
        currentPage={page}
        totalPage={totalCount}
        handleClickPage={handleClickPage}
        maxPaginationItem={MAX_PAGINATION_ITEM}
      />
      <ApiErrorMessage errorMessage={error} />
      <SearchMovieList
        page={DEFAULT_PAGE}
        count={DEFAULT_COUNT}
        getMovieListWith={getMovieListWith}
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
