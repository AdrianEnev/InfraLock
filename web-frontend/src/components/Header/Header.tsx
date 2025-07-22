'use client'
import { useRouter, usePathname } from "next/navigation";
import { useEffect } from 'react';
import { useGlobalContext } from "../GlobalContext";

function Header() {
    const router = useRouter();
    const pathname = usePathname();

    const {isAuthenticated} = useGlobalContext();

    useEffect(() => {
        // Scroll to top when path changes
        window.scrollTo(0, 0);
    }, [pathname]);

    return (
        <div className='w-full h-full px-[3%] sm:px-[5%] md:px-[10%] pb-3 pt-2 flex items-center select-none'>
            <div className="w-full h-full flex flex-row justify-around">
                <div>
                    <p className="text-lg lg:text-xl my-3 hover:opacity-60" onClick={() => router.push('/')}><span className="text-2xl">🔒</span> InfraLock API</p>
                </div>

                <div className="flex flex-row gap-x-3">
                    <p className="text-base lg:text-lg text-gray-500 my-3 hover:opacity-60" onClick={() => router.push('/pricing')}>Pricing</p>
                    <p className="text-base lg:text-lg text-gray-500 my-3 hover:opacity-60" onClick={() => router.push('/docs')}>Docs</p>
                    <p className="text-base lg:text-lg text-gray-500 my-3 hover:opacity-60">Use Cases</p>
                    <p className="text-base lg:text-lg text-gray-500 my-3 hover:opacity-60">Developers</p>
                </div>
                 
                <div className="flex flex-row gap-x-5">
                    {isAuthenticated ? (
                        <div className="flex flex-row gap-x-3">
                            <p className={`text-base text-blue-500 lg:text-lg my-3 hover:opacity-60`} onClick={() => router.push('/account')}>Account</p>

                            <button className="basic-button w-auto min-w-[128px] h-8 mt-[8px]">
                                <p className={`text-white text-base lg:text-lg my-3 hover:opacity-60`} onClick={() => router.push('/account/api')}>API</p>
                            </button>
                        </div>
                    ) : (
                        <div className="flex flex-row gap-x-3">
                            <p className={`text-base text-blue-500 lg:text-lg my-3 hover:opacity-60`} onClick={() => router.push('/login')}>Login</p>

                            <button className="basic-button w-auto min-w-[128px] h-8 mt-[8px]">
                                <p className={`text-white text-base lg:text-lg my-3 hover:opacity-60`} onClick={() => router.push('/register')}>Get Started</p>
                            </button>
                        </div>
                    )}
                   
                </div>
            </div>
        </div>
    )
}

export default Header