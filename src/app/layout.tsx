'use client' 

import './globals.css'
import {ClusterProvider} from '@/components/cluster/cluster-data-access'
import {SolanaProvider} from '@/components/solana/solana-provider'
import {UiLayout} from '@/components/ui/ui-layout'
import {ReactQueryProvider} from './react-query-provider'

const links: { label: string; path: string }[] = [
  { label: 'Clusters', path: '/clusters' },
  { label: 'Account', path: '/account' },
  { label: 'Clients', path: '/client/clientsList' },
  { label: 'Freelancers', path: '/freelancer/freelancersList' },
  { label: 'My Projects', path: '/myprojects' },
]

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <ReactQueryProvider>
          <ClusterProvider>
            <SolanaProvider>
              <UiLayout links={links}>{children}</UiLayout>
            </SolanaProvider>
          </ClusterProvider>
        </ReactQueryProvider>
      </body>
    </html>
  )
}
