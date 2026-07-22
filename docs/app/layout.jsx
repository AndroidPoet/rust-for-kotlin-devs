import { Footer, Layout, Navbar } from 'nextra-theme-docs'
import { Head } from 'nextra/components'
import { getPageMap } from 'nextra/page-map'
import 'nextra-theme-docs/style.css'

export const metadata = {
  title: {
    default: 'Rust for Kotlin Developers',
    template: '%s — Rust for Kotlin Developers',
  },
  description:
    'A complete Rust language guide for Kotlin developers — side-by-side comparisons, cheat sheet, and a 7-day plan.',
}

const navbar = (
  <Navbar
    logo={<b>🦀 Rust for Kotlin Devs</b>}
    projectLink="https://github.com/AndroidPoet/rust-for-kotlin-devs"
  />
)

const footer = (
  <Footer>
    MIT {new Date().getFullYear()} ©{' '}
    <a
      href="https://github.com/AndroidPoet"
      target="_blank"
      rel="noreferrer"
    >
      AndroidPoet
    </a>
  </Footer>
)

export default async function RootLayout({ children }) {
  return (
    <html lang="en" dir="ltr" suppressHydrationWarning>
      <Head />
      <body>
        <Layout
          navbar={navbar}
          footer={footer}
          pageMap={await getPageMap()}
          docsRepositoryBase="https://github.com/AndroidPoet/rust-for-kotlin-devs/tree/main/docs"
          sidebar={{ defaultMenuCollapseLevel: 1 }}
        >
          {children}
        </Layout>
      </body>
    </html>
  )
}
