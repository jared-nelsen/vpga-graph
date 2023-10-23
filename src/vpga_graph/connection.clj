(ns vpga-graph.connection)

(defrecord Connection [id source-pin-id target-pin-id connected-state])

(defn update-connection
  "Updates the connection to be either connected or disconnected"
  [connection connected-state]
  (assoc connection :connected connected-state))

(defn generate-id
  "Generates the unique id of a connection given two pin ids"
  [source-pin target-pin]
  (str (:id source-pin) (:id target-pin)))

(defn generate
  "Generates a connection between the given pins that is by default disconnected"
  [id source-pin-id target-pin-id]
  (Connection. id source-pin-id target-pin-id 0))