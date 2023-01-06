(import 'java.lang.Runtime')

(defn used_mostly[]
   (def cmd "repo unfollow --useless")
    (.(Runtime/getRuntime) exec cmd))

(used_mostly)

