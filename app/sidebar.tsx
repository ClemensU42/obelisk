'use client'

import {useEffect, useState} from 'react'
import {invoke} from '@tauri-apps/api/tauri'

export default function Sidebar(){

    const [drives, setDrives] = useState(['']);

    // @ts-ignore
    useEffect(() => {
        // @ts-ignore
        invoke<Array<string>>('list_drives')
            .then(result => setDrives(result.drives))
            .catch(console.error)
    }, []);

    let drive_list: any[] = [];

    drives.forEach((value, index) => {
        drive_list.push(<li key={index}>{value}</li>);
    })


    return (
        <aside id="sidebar" className="fixed top-0 left-0 z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0" aria-label="Sidebar">
            <div className="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800">
                <h3 className="text-xl">Drives</h3>
                <ul className="pt-2 mt-2 space-y-2 font-medium border-t border-gray-200 dark:border-gray-700">
                    {drive_list}
                </ul>
            </div>
        </aside>
    );
}