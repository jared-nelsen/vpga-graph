(ns vpga-graph.connection)

(defrecord Connection [id source-pin-id target-pin-id connected])

(defn generate
  "Generates a connection between the given pins that is by default disconnected"
  [source target]
  (Connection. "" source target 0))

(defn update
  "Updates the connection to be either connected or disconnected"
  [connection connected]
  (assoc connection :connected connected))

(defn generate-id
  "Generates the unique id of a connection given two pin ids"
  [source-pin target-pin]
  (str (:id source-pin) (:id target-pin)))