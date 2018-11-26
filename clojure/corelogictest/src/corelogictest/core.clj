(ns corelogictest.core
  (:refer-clojure :exclude [==])
  (:require [clojure.core.logic :refer :all  :exclude [pldb.indexed?]])
  (:require [clojure.core.logic.pldb :as pldb :refer [with-db with-dbs]])
  (:gen-class))

(def men
  (pldb/db
    [man 'Bob]
    [man 'John]))

(def fun-people
  (pldb/db
    [fun 'Bob]))

(def facts
  (pldb/db
   [woman 'Lucy]
   [woman 'Mary] ))

(def facts
  (-> facts
      (pldb/db-fact likes 'Bob 'Mary)
      (pldb/db-fact likes 'John 'Lucy)))

(def facts
  (pldb/db-fact facts likes 'Mary 'John))

(def facts
  (pldb/db-fact facts likes 'John 'Bob))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!")
  (pldb/db-rel man x)
  (pldb/db-rel woman x)
  (pldb/db-rel likes p1 p2)
  (pldb/db-rel fun x)
  (with-dbs [men fun-people]
    (run* [q]
      (man q)
      (fun q)))
  (with-dbs [men facts]
    (run* [q]
      (likes 'Mary q)))
  (with-dbs [men facts] (run* [q] (fresh [x y] (likes x y) 
                                               (== q [x y]))))
  (with-dbs [men facts] (run* [q] (fresh [x y z] (likes x y) 
                                               (likes y z)
                                               (likes z x)
                                               (== q [x y z]))))
  (pprint 3)
)

(-main)