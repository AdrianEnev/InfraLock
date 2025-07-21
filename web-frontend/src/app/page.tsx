'use client'

import Link from "next/link";


export default function Home() {
    return (
        <div className="w-screen h-screen overflow-x-hidden">
            <div className="w-full h-full flex flex-col items-center pt-[5%]">
                <p className="text-5xl">Lorem <span className="font-medium text-blue-400">Ipsum</span> 🕵</p>
                <p className="text-lg max-w-1/3 text-center mt-4 text-gray-500">Lorem ipsum dolor, sit amet consectetur adipisicing elit. Debitis in libero sapiente quasi voluptate laudantium non, sunt soluta rerum blanditiis aperiam deserunt dolorum accusantium ipsam cupiditate adipisci sequi ipsa possimus.</p>
               
                <div className="flex flex-row gap-x-3 mt-3">
                    <Link className="w-48 py-2 rounded-xl bg-blue-400 mt-4 flex items-center justify-center hover:opacity-60" href={'/pricing'}>
                        <p className="text-white text-xl">Pricing</p>
                    </Link>
                    <Link className="w-48 py-2 rounded-xl bg-blue-400 mt-4 flex items-center justify-center hover:opacity-60" href={'/docs'}>
                        <p className="text-white text-xl">Docs</p>
                    </Link>
                </div>
                
            </div>
        </div>
    );
}
