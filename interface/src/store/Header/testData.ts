interface UserProps {
  isLogin: boolean;
  name?: string;
  id?: number;
}

export const currentUser: UserProps = {
    isLogin: true,
    name: 'silin',
    id: 1
    }
interface disableFrom {
    newdisable: boolean;
    editdisable: boolean;
    logindisable: boolean;
}

export const disabled: disableFrom = {
    newdisable: false,
    editdisable: true,
    logindisable: false
}
