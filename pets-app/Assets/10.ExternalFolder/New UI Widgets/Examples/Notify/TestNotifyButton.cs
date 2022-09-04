namespace UIWidgets.Examples
{
	using UIWidgets;
	using UnityEngine;

	/// <summary>
	/// Test notification with buttons.
	/// </summary>
	public class TestNotifyButton : MonoBehaviour
	{
		/// <summary>
		/// Notification template.
		/// Gameobject in Hierarchy window, parent gameobject should have Layout component (recommended EasyLayout)
		/// </summary>
		[SerializeField]
		protected Notify NotificationTemplate;

		/// <summary>
		/// Show notification.
		/// </summary>
		public async void ShowNotify()
		{
			var actions = new NotificationButton[]
			{
				"Close",
				"Log",
			};

			var instance = NotificationTemplate.Clone().SetButtons(actions);
			var button_index = await instance.ShowAsync("Notification with buttons. Hide after 5 seconds.", customHideDelay: 5f, closeOnButtonClick: false);

			while (button_index == 1)
			{
				Debug.Log("click notification button");
				button_index = await instance;
			}

			if (button_index == 0)
			{
				Debug.Log("close notification");
			}
			else
			{
				Debug.Log("hide button or timeout");
			}

			instance.Hide();
		}

		/// <summary>
		/// Show multiple notifications.
		/// </summary>
		public async void ShowMultiple()
		{
			await NotificationTemplate.Clone().SetButtons().ShowAsync("First Notification. Hide after 5 seconds.", customHideDelay: 5f, hideAnimation: NotificationBase.AnimationCollapseHorizontal);

			await NotificationTemplate.Clone().SetButtons().ShowAsync("Second Notification. Hide after 2 seconds.", customHideDelay: 2f, hideAnimation: NotificationBase.AnimationCollapseHorizontal);

			await NotificationTemplate.Clone().SetButtons().ShowAsync("Third Notification. Hide after 1 seconds.", customHideDelay: 1f, hideAnimation: NotificationBase.AnimationCollapseHorizontal);
		}
	}
}