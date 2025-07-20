'use client'
import postUser from "@hooks/users/postUser"
import { useRouter } from "next/navigation";
import { useState } from "react"

function RegisterPage() {

    const router = useRouter();
    const [username, setUsername] = useState("")
    const [email, setEmail] = useState("")
    const [password, setPassword] = useState("")

    // Handle register form submission
    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!email || !password) {
            return;
        }

        const user = await postUser(username, email, password);
        if (user) {
            router.replace('/login')
        } else {
            console.error("Registration failed");
        }
    }

    return (
        <div className="w-screen h-screen pt-3 px-3">
            <div className="w-full h-full flex flex-col items-center justify-center mt-[-10%]">

                <p className="text-3xl font-medium">Register</p>

                <form className="w-[90%] md:w-1/3">
                    <div className="mt-6">
                        <label className="block text-sm font-medium ">Username</label>
                        <input type="text" className="mt-1 px-3 py-2 block w-full border border-gray-300 rounded-md shadow-sm focus:ring-red-400 focus:border-red-400 sm:text-sm" 
                            onChange={(e) => setUsername(e.target.value)}
                        />
                    </div>

                    <div className="mt-4">
                        <label className="block text-sm font-medium ">Email</label>
                        <input type="email" className="mt-1 px-3 py-2 block w-full border border-gray-300 rounded-md shadow-sm focus:ring-red-400 focus:border-red-400 sm:text-sm" 
                            onChange={(e) => setEmail(e.target.value)}
                        />
                    </div>

                    <div className="mt-4">
                        <label className="block text-sm font-medium ">Password</label>
                        <input type="password" className="mt-1 px-3 py-2 block w-full border border-gray-300 rounded-md shadow-sm focus:ring-red-400 focus:border-red-400 sm:text-sm" 
                            onChange={(e) => setPassword(e.target.value)}
                        />
                    </div>

                    <button type="submit" className="mt-6 w-full bg-red-400 text-white font-semibold py-2 px-4 rounded-md hover:opacity-60"
                        onClick={handleSubmit}
                    >
                        Register
                    </button>
                </form>
            </div>
        </div>
    )
}

export default RegisterPage