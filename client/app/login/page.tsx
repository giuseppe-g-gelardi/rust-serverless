import LoginForm from "~/components/login_form";

export default async function Login() {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen py-2">
      This is the login page
      <LoginForm />
    </div>
  );
}
