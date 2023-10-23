(ns vpga-graph.util)

(defn ->UUID
  "Generates a new UUID"
  []
  (str (java.util.UUID/randomUUID)))