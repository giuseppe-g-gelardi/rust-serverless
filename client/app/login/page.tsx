import { register_user } from "./register_user";

export default async function Login() {
  async function testSubmit(formData: FormData) {
    "use server";

    const authForm = {
      email: formData.get("email"),
      password: formData.get("password"),
    };

    try {
      const response = await register_user({
        email: authForm.email as string,
        password: authForm.password as string,
      });
      return response;
    } catch (error: unknown) {
      console.log("error", error);
      return error;
    }
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen py-2">
      <div className="flex min-h-full flex-1 flex-col justify-center px-6 py-12 lg:px-8 bg-white">
        <div className="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
          <form action={testSubmit} className="space-y-6">
            <div>
              <label
                htmlFor="email"
                className="block text-sm font-medium leading-6 text-gray-500"
              >
                Email address
              </label>
              <div className="mt-2">
                <input
                  id="email"
                  name="email"
                  type="email"
                  required
                  className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
              </div>
            </div>

            <div>
              <div className="flex items-center justify-between">
                <label
                  htmlFor="password"
                  className="block text-sm font-medium leading-6 text-gray-500"
                >
                  Password
                </label>
              </div>
              <div className="mt-2">
                <input
                  id="password"
                  name="password"
                  type="password"
                  required
                  className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
              </div>
            </div>

            <div>
              <button
                type="submit"
                className="flex w-full justify-center rounded-md bg-emerald-400/50 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
              >
                Sign in
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}

// export default function LoginForm() {
//   const [email, setEmail] = useState<string>("");
//   const [password, setPassword] = useState<string>("");
//
//   const handleRegister = async (e: FormEvent) => {
//     e.preventDefault();
//
//     try {
//       await register_user({ email, password });
//     } catch (e) {
//       console.error("Error: ", e);
//     }
//   };

// return (
// );
// }

//   <form onSubmit={handleRegister}>
//     <label>
//       First Name:
//       <input
//         className="text-red-400"
//         onChange={(e) => setEmail((e.target as HTMLInputElement).value)}
//         type="email"
//         name="email"
//       />
//     </label>
//     <br />
//     <label>
//       Last Name:
//       <input
//         onChange={(e) => setPassword((e.target as HTMLInputElement).value)}
//         type="password"
//         name="password"
//       />
//     </label>
//     <br />
//
//     {/* Add more fields as needed */}
//
//     <button type="submit">Submit</button>
//   </form>
// );
// }

// import LoginForm from "~/components/login_form";
//
// export default async function Login() {
//   return (
//     <div className="flex flex-col items-center justify-center min-h-screen py-2">
//       This is the login page
//       <LoginForm />
//     </div>
//   );
// }
