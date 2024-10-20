"use client"
import style from "./style.module.css";
import { SubmitHandler, useForm } from "react-hook-form";
import TextBox from "@/components/form/TextBox";
import TextBoxArea from "@/components/form/TextArea";
import postCreate from "@/api/post";
type MovieCreationForm = {
    movieTitle: string;
    movieContents: string;
    movieMemo: string;
};
export const Create = () => {
    const form = useForm<MovieCreationForm>();
    const { handleSubmit } = form;
    const handleSubmitCreateButton: SubmitHandler<MovieCreationForm> = (submitForm: MovieCreationForm) => {
        const {
            movieTitle,
            movieMemo,
            movieContents
        } = submitForm;
        const request = {
            "title": movieTitle,
            "description": movieContents,
            "memo": movieMemo,
            "session_token": "aaaaa", // 仮状態で適当な文字列に設定
        };
        const response = postCreate(request);
        console.log(response);
    }

    return (
            <form className={style["create-form"]} onSubmit={handleSubmit(handleSubmitCreateButton)} >
                <TextBox title="映画タイトル" name="movieTitle" form={form} />
                <TextBoxArea title="映画情報" name="movieContents" form={form} />
                <TextBoxArea title="メモ" name="movieMemo" form={form} />
                <div className={style["create-form-button"]}>
                    <button id="createButton" type="submit">登録</button>
                </div>
            </form>
    )
}
export default Create;
