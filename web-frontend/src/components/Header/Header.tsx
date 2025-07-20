'use client'
import { useRouter } from "next/navigation";

function Header() {

    const router = useRouter();

    return (
        <div className='w-full h-full px-[3%] sm:px-[5%] md:px-[10%] pb-3 pt-5 flex items-center select-none'>
            <div className="w-full h-full flex flex-row justify-between">
                <p className="text-2xl lg:text-3xl text-red-400 font-medium my-3 hover:opacity-60" onClick={() => router.push('/')}>User Behaviour API</p>
                 
                <div className="flex flex-row gap-x-5">
                    <p className={`text-xl lg:text-2xl my-3 text-white hover:opacity-60`} onClick={() => router.push('/docs')}>Docs</p>
                    <p className={`text-xl lg:text-2xl my-3 text-white hover:opacity-60`} onClick={() => router.push('/account')}>Account</p>
                </div>
            </div>
        </div>
    )
}

export default Header