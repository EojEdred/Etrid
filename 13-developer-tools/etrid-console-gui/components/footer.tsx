import { Github, Twitter, MessageCircle, Send } from "lucide-react"

const footerLinks = {
  main: ["About", "Docs", "Whitepaper", "GitHub", "Blog"],
  social: [
    { name: "Twitter", icon: Twitter },
    { name: "Discord", icon: MessageCircle },
    { name: "Telegram", icon: Send },
    { name: "GitHub", icon: Github },
  ],
}

export default function Footer() {
  return (
    <footer className="border-t border-border bg-background">
      <div className="container mx-auto px-4 py-12">
        <div className="flex flex-col md:flex-row justify-between items-center gap-8">
          {/* Logo */}
          <div className="text-3xl font-bold">ËTRID</div>

          {/* Links */}
          <nav className="flex flex-wrap justify-center gap-6">
            {footerLinks.main.map((link) => (
              <a key={link} href="#" className="text-muted-foreground hover:text-foreground transition-colors">
                {link}
              </a>
            ))}
          </nav>

          {/* Social Icons */}
          <div className="flex gap-4">
            {footerLinks.social.map((social) => {
              const Icon = social.icon
              return (
                <a
                  key={social.name}
                  href="#"
                  className="w-10 h-10 rounded-full bg-muted hover:bg-primary transition-colors flex items-center justify-center"
                  aria-label={social.name}
                >
                  <Icon className="w-5 h-5" />
                </a>
              )
            })}
          </div>
        </div>

        {/* Copyright */}
        <div className="mt-8 pt-8 border-t border-border text-center text-sm text-muted-foreground">
          © 2026 Ëtrid. MIT License.
        </div>
      </div>
    </footer>
  )
}
