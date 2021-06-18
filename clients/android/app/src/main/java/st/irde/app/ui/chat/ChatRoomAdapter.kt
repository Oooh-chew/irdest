package st.irde.app.ui.chat

import android.util.Log
import android.view.View
import android.view.ViewGroup
import androidx.recyclerview.widget.RecyclerView
import kotlinx.android.synthetic.main.item_chat_message_received.view.*
import kotlinx.android.synthetic.main.item_chat_message_sent.view.chat_message_body
import kotlinx.android.synthetic.main.item_chat_message_sent.view.chat_message_time
import st.irde.app.R
import st.irde.app.ffi.models.ChatMessage
import st.irde.app.util.AppState
import st.irde.app.util.inflate
import java.lang.Exception

const val MESSAGE_TYPE_SENT: Int = 1
const val MESSAGE_TYPE_RECV: Int = 2

class ChatRoomAdapter(private val messages: MutableList<ChatMessage>)
    : RecyclerView.Adapter<ChatRoomAdapter.MsgHolder>() {

    fun addMessage(msg: ChatMessage) {
        messages.add(msg)
    }

    override fun getItemViewType(position: Int): Int {
        if (messages[position].author == AppState.self) {
            return MESSAGE_TYPE_SENT
        } else {
            return MESSAGE_TYPE_RECV
        }
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): MsgHolder {
        when (viewType){
            MESSAGE_TYPE_SENT -> {
                val inflated = parent.inflate(R.layout.item_chat_message_sent)
                return MsgHolder(inflated, false)
            }
            MESSAGE_TYPE_RECV -> {
                val inflated = parent.inflate(R.layout.item_chat_message_received)
                return MsgHolder(inflated, true)
            }
            else -> throw Exception("Invalid ViewType for message!  Not 1 or 2")
        }
    }

    override fun getItemCount() = messages.size

    override fun onBindViewHolder(holder: MsgHolder, position: Int) {
        holder.bindMessage(messages[position])
    }

    class MsgHolder(v: View, private val received: Boolean)
        : RecyclerView.ViewHolder(v), View.OnClickListener {
        private var view: View = v
        lateinit var msg: ChatMessage

        init {
            v.setOnClickListener(this)
        }

        fun bindMessage(msg: ChatMessage) {
            this.msg = msg

            view.chat_message_body.text = msg.content!!
            view.chat_message_time.text = msg.timestamp!!

            if (received) {
                view.chat_message_name.text = AppState.getUserProfile(msg.author)?.handle
            }
        }

        override fun onClick(v: View?) {
            Log.d("chatroom", "We might want to do something with long-presses")

        }

        companion object {
            private const val ROOM_KEY = "ROOM"
        }
    }
}
