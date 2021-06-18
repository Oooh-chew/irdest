package st.irde.app.ui.users

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.fragment.app.FragmentManager
import androidx.fragment.app.FragmentStatePagerAdapter
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import androidx.viewpager.widget.ViewPager
import com.google.android.material.tabs.TabLayout
import st.irde.app.R
import st.irde.app.ffi.models.UserProfile
import st.irde.app.util.AppState

class UsersFragment : Fragment() {

    private lateinit var adapter: UsersTabsAdapter
    private lateinit var pager: ViewPager

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, bundle: Bundle?): View? {
        return inflater.inflate(R.layout.fragment_users, container, false)
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        adapter = UsersTabsAdapter(fragmentManager!!)
        pager = view.findViewById(R.id.users_tab_pager)
        pager.adapter = adapter

        val layout = view.findViewById<TabLayout>(R.id.users_tab_layout)
        layout.setupWithViewPager(pager)
    }
}

class UsersTabsAdapter(val fm: FragmentManager)
    : FragmentStatePagerAdapter(fm, BEHAVIOR_RESUME_ONLY_CURRENT_FRAGMENT) {
    override fun getItem(position: Int): Fragment {
        if (position == 1) {
            return UsersListFragment(true, fm)
        } else {
            return UsersListFragment(false, fm)
        }
    }

    override fun getPageTitle(position: Int): CharSequence? {
        if(position == 1) {
            return "All Users"
        } else {
            return "Friends"
        }
    }

    override fun getCount(): Int = 2
}

class UsersListFragment(val friends: Boolean, val fm: FragmentManager) : Fragment() {
    private lateinit var list: RecyclerView
    val users: MutableList<UserProfile> = mutableListOf()

    // This function should be called whenever the view is updated
    fun updateUsers() {
        users.clear()
        if (!friends) {
            for(u in AppState.get().usersList(false)) {
                users.add(u)
            }
        }

        list.adapter = UsersListAdapter(users, fm)
    }

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, savedInstanceState: Bundle?): View? {
        val root = inflater.inflate(R.layout.fragment_users_list, container, false)
        list = root!!.findViewById<RecyclerView>(R.id.users_list)!!
        list.layoutManager = LinearLayoutManager(context)
        updateUsers()
        return root
    }
}
