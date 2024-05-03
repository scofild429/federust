import axios from "axios";

axios.defaults.withCredentials = true; 
const setAuthHeader = (token: string) => {
  if(token) {
    axios.defaults.headers.common = {
      Authorization: token,
    };
  } else {
    delete axios.defaults.headers.common.Authorization;
  }
};

export default setAuthHeader;
