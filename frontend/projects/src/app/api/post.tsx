
import { BASE_API_URL,REGISTER_POST_PATH } from "@/constants/constants";
import axios from "axios";

type PostCreateMovieRequest = {
    title:String,
    description:String,
    memo:String,
    session_token:String
};
const postCreateForm = async (request : PostCreateMovieRequest) =>{
    const postUrl = BASE_API_URL + REGISTER_POST_PATH;
    try{
        const response = await axios.post(postUrl,request);
        return response;
    }catch(error){
        return error;
    }
}
export default postCreateForm;