"use client";
import SearchMovieList from "@/components/SearchMovieList";
import ViewMovieList from "@/components/ViewMovieList";

import style from "./style.module.css";

export const MovieList = () => {
  return (
    <div className={style["movie-list-container"]}>
      <SearchMovieList />
      <ViewMovieList />
    </div>
  );
};
export default MovieList;
