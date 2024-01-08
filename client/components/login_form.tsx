"use client";

import { FormEvent, useState } from "react";

type RegisterForm = {
  email: string;
  password: string;
};

async function register_user<T>(form: RegisterForm): Promise<T> {
  const res = await fetch("https://rust-serverless.vercel.app/api/auth/login", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Access-Control-Allow-Origin": "*",
      Cookie: "*",
    },
    body: JSON.stringify(form),
  });

  if (!res.ok) {
    throw new Error("Failed to register user");
  }

  console.log(res.json());

  return res.json();
}

export default function LoginForm() {
  const [email, setEmail] = useState<string>("");
  const [password, setPassword] = useState<string>("");

  const handleRegister = async (e: FormEvent) => {
    e.preventDefault();

    try {
      await register_user({ email, password } as RegisterForm);
    } catch (e) {
      console.log(e);
    }
  };

  return (
    <div className="flex min-h-full flex-1 flex-col justify-center px-6 py-12 lg:px-8 bg-white">
      <div className="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
        <form onSubmit={handleRegister} className="space-y-6">
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
                // autoComplete="email"
                onChange={(e) => setEmail((e.target as HTMLInputElement).value)}
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
                onChange={(e) =>
                  setPassword((e.target as HTMLInputElement).value)
                }
                // autoComplete="current-password"
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
  );
}

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
