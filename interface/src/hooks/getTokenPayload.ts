import jwt_decode from 'jwt-decode';

const getTokenPayload = (token:string) => {
	if (!token) {
		return null;
	}

  const tokenClaims: any = jwt_decode(token);

	if (Date.now() / 1000 >= tokenClaims.exp) {
		return null;
	}

	return tokenClaims;
};

export default getTokenPayload
