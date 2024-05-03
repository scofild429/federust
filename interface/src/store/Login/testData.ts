// interface EmailRef {
//     val: string;
//     error: boolean;
//     message: string;
// }

// export const emailRef: EmailRef = {
//     val: '',
//     error: false,
//     message: ''
// }
export const emailReg = /^([\w-_]+(?:\.[\w-_]+)*)@((?:[a-z0-9]+(?:-[a-zA-Z0-9]+)*)+\.[a-z]{2,6})$/
interface RuleProp {
  type: 'required' | 'email' | 'passlength';
  message: string;
}
type RulesProp = RuleProp[]
export const emailRules: RulesProp = [
    { type: 'required', message: 'Email address can not be empty' },
    { type: 'email', message: 'Please give your validated  email addresse' }
]

export const passwordRules: RulesProp = [
    { type: 'required', message: 'Email address can not be empty' },
    { type: 'passlength', message: 'Password must longer than 3' }
]

interface getpass {
    pass: boolean
}
export const GetPass: boolean [] = []
